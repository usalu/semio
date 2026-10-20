// 🧽 Removes the re-exports a lifted module inherited from its former script and points their importers at the owning module instead.
import {LIB, Vfs} from "../vfs.ts";

const E = (cp: number): string => String.fromCodePoint(cp) + "️";
const TS = E(0x1F7E6) + ".ts", MJS = E(0x1F7E8) + ".mjs";
const BUILD = `${LIB}/${E(0x1F3D7)}build`, INPUTS = `${BUILD}/${E(0x1F4E5)}inputs`;

export function planShims(v: Vfs, log: (row: Record<string, unknown>) => void): void {
  const digest = `${INPUTS}/${E(0x1F9EE)}digest/${TS}`, lock = `${INPUTS}/${E(0x1F512)}lock/${TS}`;
  v.edit(digest, `export{sourceClosureDigest}from"./${MJS}";\n`, "");
  v.edit(lock, `export{selectedCargoLockDigest}from"./${E(0x1F4E5)}read/${MJS}";\n`, "");
  const buildMode = `${LIB}/${E(0x26A1)}caching/${E(0x1F9EA)}tests/${E(0x1F39A)}build-mode/${TS}`;
  const digestPath = `../../../${E(0x1F3D7)}build/${E(0x1F4E5)}inputs/${E(0x1F9EE)}digest`;
  v.edit(buildMode, `import { sourceClosureDigest } from "${digestPath}/${TS}";\n`, "");
  v.edit(buildMode, `import { sourceClosureTaskEnvironments, sourceClosureReceiptHolds } from "${digestPath}/${MJS}";`, `import { sourceClosureDigest, sourceClosureTaskEnvironments, sourceClosureReceiptHolds } from "${digestPath}/${MJS}";`);
  log({kind: "shim-removal", removed: ["sourceClosureDigest", "selectedCargoLockDigest"], importers: [buildMode.replace(LIB, "L")]});
}
