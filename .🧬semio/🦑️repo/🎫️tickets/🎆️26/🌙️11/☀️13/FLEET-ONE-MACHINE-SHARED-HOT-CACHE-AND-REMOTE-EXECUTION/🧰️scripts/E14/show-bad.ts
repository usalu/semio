import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const p=process.argv[2]!,rows=JSON.parse(brotliDecompressSync(readFileSync(p)).toString());
console.log(JSON.stringify(rows[Number(process.argv[3])]).slice(0,500));
