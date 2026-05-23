import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const fixturePath = join(import.meta.dirname, "../../../../../../.storybook/fixtures/nakagin-capsule-tower.board.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
let migrated = 0;
for (const node of fixture.nodes ?? []) {
	if (typeof node.label === "string" && node.label.trim() !== "") {
		if (!node.text) node.text = node.label;
		delete node.label;
		migrated += 1;
	}
}
writeFileSync(fixturePath, `${JSON.stringify(fixture, null, 2)}\n`, "utf8");
console.log(`[DEBUG] migrated ${migrated} board node labels to text in ${fixturePath}`);
