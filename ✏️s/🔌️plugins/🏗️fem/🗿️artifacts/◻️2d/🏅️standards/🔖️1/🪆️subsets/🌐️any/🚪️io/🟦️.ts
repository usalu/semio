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
 * `🗿️artifacts/**/📚️examples/**/🧪️tests/🟦️.ts` — no test under `🚪️io/` is reachable, so such a
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
  /** 🚫️ `false` when the hop is registered only to hand the caller a typed reason for refusing. */
  readonly implemented: boolean;
}

/** 🎯️ This subset's own dialect. */
export const FEM2D_DIALECT: IoDialect = { artifactKind: "s.fem.fem2d", standard: "1", subset: "*" };

const foreign = (artifactKind: string, standard: string): IoDialect => ({ artifactKind, standard, subset: "*" });

const TXT = foreign("s.stdio.txt", "utf-8");
const JSON_RFC8259 = foreign("s.stdio.json", "rfc8259");
const CSV = foreign("s.stdio.csv", "rfc4180");
const MD = foreign("s.stdio.md", "commonmark");
const STL = foreign("s.stdio.stl", "ascii");
const OBJ = foreign("s.stdio.obj", "3.0");

/**
 * 🚪️ The twelve rows `io()` registers, in the same order the Rust `entries()` literal lists them.
 * `stl`/`obj` export real geometry from the meshing kernel; their import direction is registered at
 * the weakest fidelity purely so a caller routed there gets a reason instead of a bare "no route".
 */
export const FEM2D_IO_ENTRIES: readonly IoEntryTwin[] = [
  { from: FEM2D_DIALECT, into: TXT, fidelity: "exact", implemented: true },
  { from: TXT, into: FEM2D_DIALECT, fidelity: "exact", implemented: true },
  { from: FEM2D_DIALECT, into: JSON_RFC8259, fidelity: "exact", implemented: true },
  { from: JSON_RFC8259, into: FEM2D_DIALECT, fidelity: "exact", implemented: true },
  { from: FEM2D_DIALECT, into: CSV, fidelity: "exact", implemented: true },
  { from: CSV, into: FEM2D_DIALECT, fidelity: "exact", implemented: true },
  { from: FEM2D_DIALECT, into: MD, fidelity: "exact", implemented: true },
  { from: MD, into: FEM2D_DIALECT, fidelity: "exact", implemented: true },
  { from: FEM2D_DIALECT, into: STL, fidelity: "lossy", implemented: true },
  { from: STL, into: FEM2D_DIALECT, fidelity: "lossy", implemented: false },
  { from: FEM2D_DIALECT, into: OBJ, fidelity: "lossy", implemented: true },
  { from: OBJ, into: FEM2D_DIALECT, fidelity: "lossy", implemented: false },
] as const;

/** 🏷️ The single column the `s.stdio.csv` envelope declares — mirrors the Rust leaves' `PAYLOAD_COLUMN`. */
export const CSV_PAYLOAD_COLUMN = "payload";

/** 🏷️ The info string the `s.stdio.md` envelope's fenced block carries — mirrors the Rust leaves' `FENCE_INFO`. */
export const MD_FENCE_INFO = "fem2d";

/** 📄️ The `.semio` text preamble every `fem2d` document opens with — mirrors the Rust `DSL_PREAMBLE`. */
export const DSL_PREAMBLE = "semio fem.fem2d.dsl ";
