// 🧪️ keys3-g law: the `SEMIO_CARGO_LOCK_*` token that keys cargo-backed script targets (typegen, generate, the proctor test) carries what cargo reads and nothing else, on the REAL repository files.
// The token's value is `cargoResolutionDigestInBatchV1` = composeCargoResolutionV1(lock slice, workspace identity, toolchain pin, cargo configuration) (🏗️build/📥️inputs/🧮️digest taskEnvironments). A file oracle (Seatbelt) sees `Cargo.lock`, `rust-toolchain.toml` and `.cargo/config.toml` as undeclared files because the key names them by MEANING, not by path; this law proves the meaning moves exactly when cargo's behaviour can move:
//   toolchain channel edit moves it, a comment-only edit does not; a cargo config edit (env/rustflags) moves it, an artifact-location edit (`build.target-dir`) does not;
//   a version bump of a lock entry INSIDE the crate's dependency closure moves the lock part, a bump of an entry OUTSIDE it does not (independent oracle: `cargo-closure.py`, a line reader over the real Cargo.lock, no repository code).
// Graph-free, reads the live files, never writes them (overlay host). Run: bun test ./<this file>
import {describe, expect, test} from "bun:test";
import {readFileSync, existsSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {spawnSync} from "node:child_process";

const LIVE = "/Users/ueli/Documents/semio";
const R = LIVE + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🦀️resolution";
const {cargoResolutionPartsV1, composeCargoResolutionV1, createCargoResolutionContextV1, readCargoResolutionPolicyV1} = await import(R + "/🟦️.ts");
const policy = readCargoResolutionPolicyV1(JSON.parse(readFileSync(R + "/🔣️.json", "utf8")));

const CRATES = {
  "framework-actor-rs typegen": "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust",
  "framework-rs generate": "🧰️framework/📦️packages/🦀️rust",
  "ui-contract-rs generate": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust",
  "proctor (architecture-course test)": "🎓️teaching/🛂️proctor/📦️packages/🦀️rust",
} as const;

type Overlay = Map<string, string>;
const files = new Map<string, string>();
const fileText = (path: string): string | null => { if (!files.has(path)) files.set(path, existsSync(join(LIVE, path)) ? readFileSync(join(LIVE, path), "utf8") : ""); return files.get(path)! || null; };
const sha = (text: string) => createHash("sha256").update(text).digest("hex");
const host = (overlay: Overlay) => ({
  read: async (path: string) => { const text = overlay.get(path) ?? fileText(path); return text === null ? null : {value: Bun.TOML.parse(text) as Record<string, unknown>, sha: sha(text)}; },
  digest: async (path: string) => { const text = overlay.get(path) ?? fileText(path); return text === null ? null : sha(text); },
});
const parts = (crate: string, overlay: Overlay) => cargoResolutionPartsV1(createCargoResolutionContextV1(host(overlay), policy), crate + "/Cargo.toml");
const oracle = (crate: string) => JSON.parse(spawnSync("python3", ["-I", join(import.meta.dir, "cargo-closure.py"), LIVE, crate], {encoding: "utf8", maxBuffer: 16 * 1024 * 1024}).stdout) as {lock: string; package: string; closureSize: number; inside: string[]; outside: string[]};
const bump = (lock: string, name: string) => {
  const at = lock.indexOf(`[[package]]\nname = "${name}"\n`);
  if (at < 0) throw new Error("lock entry not found: " + name);
  const version = /version = "([^"]+)"/u.exec(lock.slice(at, at + 400))!;
  return lock.slice(0, at) + lock.slice(at).replace(version[0], `version = "${version[1]}-keys3"`);
};

describe("the cargo resolution token on the real repository", () => {
  for (const [label, crate] of Object.entries(CRATES)) {
    describe(label, () => {
      const facts = oracle(crate);
      test("the oracle found the crate's lock and a non-trivial closure", () => {
        expect(facts.closureSize).toBeGreaterThan(20);
        expect(facts.inside.length).toBeGreaterThan(0);
      });

      test("the token is a pure function of the real files (same files, same digest)", async () => {
        const [a, b] = [await parts(crate, new Map()), await parts(crate, new Map())];
        expect(composeCargoResolutionV1(a)).toBe(composeCargoResolutionV1(b));
      });

      test("toolchain pin: a channel edit moves it, a comment-only edit does not", async () => {
        const base = await parts(crate, new Map());
        const pin = fileText("rust-toolchain.toml")!;
        const moved = await parts(crate, new Map([["rust-toolchain.toml", pin.replace(/channel = "([^"]+)"/u, 'channel = "$1-keys3"')]]));
        const same = await parts(crate, new Map([["rust-toolchain.toml", "# keys3 comment\n" + pin]]));
        expect(moved.toolchain).not.toBe(base.toolchain);
        expect(composeCargoResolutionV1(moved)).not.toBe(composeCargoResolutionV1(base));
        expect(same).toEqual(base);
      });

      test("cargo configuration: an env edit moves it, an artifact-location edit does not", async () => {
        const base = await parts(crate, new Map());
        const config = fileText(".cargo/config.toml")!;
        const moved = await parts(crate, new Map([[".cargo/config.toml", /^\[env\]$/mu.test(config) ? config.replace(/^\[env\]$/mu, '[env]\nKEYS3_PROBE = "1"') : config + '\n[env]\nKEYS3_PROBE = "1"\n']]));
        const location = /^target-dir\s*=\s*"[^"]*"/mu.test(config) ? config.replace(/^(target-dir\s*=\s*)"[^"]*"/mu, '$1"keys3-probe"') : /^\[build\]$/mu.test(config) ? config.replace(/^\[build\]$/mu, '[build]\ntarget-dir = "keys3-probe"') : config + '\n[build]\ntarget-dir = "keys3-probe"\n';
        const located = await parts(crate, new Map([[".cargo/config.toml", location]]));
        expect(moved.config).not.toBe(base.config);
        expect(located).toEqual(base);
      });

      test("lock slice: a bump inside the crate's dependency closure moves it, a bump outside does not (independent closure oracle)", async () => {
        const base = await parts(crate, new Map());
        const lock = fileText(facts.lock)!;
        for (const name of facts.inside.slice(0, 2)) expect((await parts(crate, new Map([[facts.lock, bump(lock, name)]]))).lock, name).not.toBe(base.lock);
        for (const name of facts.outside.slice(0, 2)) expect((await parts(crate, new Map([[facts.lock, bump(lock, name)]]))).lock, name).toBe(base.lock);
      });
    });
  }
});
