/**
 * 🚪️ Typed twin of `🚪️io/🦀️.rs`'s `io() -> IoDeclaration` for `s.fem.fem2d@1/*`.
 *
 * The Rust side owns the codecs; this file owns the DECLARATION — the exact `(from, into, fidelity)`
 * rows `io()` registers on the framework's `io_mechanism`, so a TypeScript caller can answer "which
 * conversions does fem2d offer, and how lossy is each" without instantiating the WASM component,
 * and so a drift between the two tables is visible by reading them side by side.
 *
 * No codec bodies here on purpose. A TS json writer or `.semio` DSL reader would have to reproduce
 * `pack::json`'s number-lexeme rule and the DSL grammar's layout rules byte for byte, and this
 * package's vitest config (`📦️packages/🟦️typescript/🧪️tests/🟦️.ts`) only discovers
 * `🗿️artifacts/**\/📚️examples/**\/🧪️tests/🟦️.ts` — no test under `🚪️io/` is reachable, so such a
 * mirror could not be pinned against the Rust output by any runnable parity test. See the ticket's
 * `📓️w4-io.md`.
 */

/** 📏️ How much of the snapshot a hop preserves — mirrors `semio_framework::io_schema::IoFidelity`. */
export type IoFidelity = "exact" | "canonical" | "semantic" | "lossy";

/** 🎯️ A dialect coordinate, `<artifactKind>@<standard>/<subset>`. */
export interface IoDialect {
  readonly artifactKind: string;
  readonly standard: string;
  readonly subset: string;
}

/** 🧾️ One directed hop, mirroring `semio_framework::io::io_mechanism::IoEntry`. */
export interface IoEntryTwin {
  readonly from: IoDialect;
  readonly into: IoDialect;
  readonly fidelity: IoFidelity;
}

/** 🎯️ This subset's own dialect. */
export const FEM2D_DIALECT: IoDialect = { artifactKind: "s.fem.fem2d", standard: "1", subset: "*" };

const foreign = (artifactKind: string, standard: string): IoDialect => ({ artifactKind, standard, subset: "*" });

const TXT = foreign("s.stdio.txt", "utf-8");
const JSON_RFC8259 = foreign("s.stdio.json", "rfc8259");
const CSV = foreign("s.stdio.csv", "rfc4180");
const STL = foreign("s.stdio.stl", "ascii");
const OBJ = foreign("s.stdio.obj", "3.0");

/**
 * 🚪️ The seven rows `io()` registers, in the same order the Rust `entries()` literal lists them.
 * `stl`/`obj` export real geometry from the meshing kernel; there is no geometry import.
 */
export const FEM2D_IO_ENTRIES: readonly IoEntryTwin[] = [
  { from: FEM2D_DIALECT, into: TXT, fidelity: "exact" },
  { from: TXT, into: FEM2D_DIALECT, fidelity: "exact" },
  { from: FEM2D_DIALECT, into: JSON_RFC8259, fidelity: "exact" },
  { from: JSON_RFC8259, into: FEM2D_DIALECT, fidelity: "exact" },
  { from: FEM2D_DIALECT, into: CSV, fidelity: "lossy" },
  { from: FEM2D_DIALECT, into: STL, fidelity: "lossy" },
  { from: FEM2D_DIALECT, into: OBJ, fidelity: "lossy" },
] as const;

/** 📄️ The `.semio` text preamble every `fem2d` document opens with — mirrors the Rust `DSL_PREAMBLE`. */
export const DSL_PREAMBLE = "semio fem.fem2d.dsl ";
