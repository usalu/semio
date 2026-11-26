import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { head, tail, concat } from "lodash";

let events: string[] = [];
for (const line of readFileSync(process.argv[2], "utf8").trim().split("\n")) {
  if (line.startsWith("#")) continue;
  const [incoming, consume, selected, waiting] = line.split(",");
  if (incoming !== "-") events = concat(events, incoming);
  assert.equal(head(events) ?? "-", selected);
  if (consume === "yes") events = tail(events);
  assert.equal(events.length, Number(waiting));
}
console.log("Six event deliveries matched Lodash head, tail and concat");
