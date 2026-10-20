// 🧾️ Prints the repository's own Nx output projection of the three report documents (cross-check for verify-independent.py).
import { printArtifactNxOutputV1 } from "../../../../../../../../🧰️framework/🛍️products/📓️print/📦️artifacts/🟦️.ts";
const live = process.argv[2] ?? "/Users/ueli/Documents/semio";
for (const id of ["zwischenbericht", "forschungsbericht", "kompaktbericht"]) console.log(printArtifactNxOutputV1(live, { version: 1, kind: "document", mode: "prototype", namespace: "report", id }));
