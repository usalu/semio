/** 📥️ Captures the authored Draw PDF bytes at the existing independent oracle boundary. */
import {writeFileSync} from "node:fs";
import {fileURLToPath} from "node:url";

const directory = new URL("./🗑️generated/", import.meta.url);
const spawn = Bun.spawn.bind(Bun);
let invocation = 0;
Bun.spawn = ((args: string[], options: {stdin: Uint8Array}) => {
    if (Array.isArray(args) && /pdftoppm(?:\.exe)?$/i.test(String(args[0]))) {
        writeFileSync(fileURLToPath(new URL("pdfium-input-" + invocation++ + ".pdf", directory)), options.stdin);
    }
    return spawn(args, options);
}) as typeof Bun.spawn;
