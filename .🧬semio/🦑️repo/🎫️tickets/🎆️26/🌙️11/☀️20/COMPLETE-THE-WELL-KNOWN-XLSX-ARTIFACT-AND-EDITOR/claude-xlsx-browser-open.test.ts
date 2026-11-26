import {expect, test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";

const root = "/Users/ueli/Documents/semio";
const shellHost = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost");
const { localArtifactAdmissionV1 } = await import(join(shellHost, "🗂️local-catalog/📥️artifact/🟦️.ts"));
const input = { documentId: "doc-1", schema: "s.stdio.xlsx@ecma-376/transitional", name: "a.xlsx", surface: { pluginId: "stdio-office", appId: "s.stdio.xlsx@ecma-376/transitional#editor" }, archive: { parent_pack: new Uint8Array([1]), parent_spr: new Uint8Array([2]), members: [] } };

/** 💾️ A browser pane keeps an opened workbook through the dev server's folder route, which needs the baked `S_DATA_DIR`; without it the open is refused with `no-data-folder`. */
test("keeping an opened workbook refuses without a data folder and admits with one", async () => {
  expect(await localArtifactAdmissionV1(input, undefined, 1)).toEqual({ refusal: "no-data-folder" });
  const admitted = await localArtifactAdmissionV1(input, "/data", 1);
  expect(admitted).toMatchObject({ binding: { kind: "folder", dataClass: "persistedLocalOnly" } });
  expect(admitted.document.target.startsWith("/data/os/local-documents/")).toBe(true);
});

test("the pane serve recipe always gives the shell a data folder short enough for the storage suffix", () => {
  const script = readFileSync(join(import.meta.dir, "claude-fulls-serve.sh"), "utf8");
  expect(script).toMatch(/export S_DATA_DIR="\$\{S_DATA_DIR:-\$R\/\.cache\/pane-data\/\$VARIANT-\$PORT\}"/u);
  expect(`${root}/.cache/pane-data/stdio-xlsx-6272/os/local-documents/${"0".repeat(64)}/.semio/.genesis-4294967295-ffffffffffffffff.stage`.length).toBeLessThan(256);
});
