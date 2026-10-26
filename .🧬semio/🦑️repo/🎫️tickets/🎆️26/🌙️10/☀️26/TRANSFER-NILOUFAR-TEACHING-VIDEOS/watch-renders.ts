import { watch } from "node:fs";
import { readFile } from "node:fs/promises";

for (const path of process.argv.slice(2)) {
    let count = 0;
    const update = async () => {
        const lines = (await readFile(path, "utf8")).split(/\r?\n/);
        const fresh = lines.slice(count).filter(line => /^\[(?:cooling|energy_balance)\] \d+\/\d+|Traceback|Error:|failed/i.test(line));
        count = lines.length - 1;
        for (const line of fresh) console.log(`[DEBUG] ${line}`);
    };
    await update();
    watch(path, () => { void update(); });
}
