/**
 * One-shot: align `compose/assets/fixtures/metabolism.kit.compose.json` bundle graph shell
 * with `metabolism.kit.light.compose.json` (wip key order, no drafts, theKit present,
 * no checkpoint frozenRoot; authoritative/stage keep minimal graph heads + theKit, not drafts).
 */
import fs from "fs";
import path from "path";

const STUB =
  "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
const root = path.resolve(
  import.meta.dirname,
  "..",
  "..",
  "..",
  "..",
  "..",
  "..",
  "compose",
  "assets",
  "fixtures",
  "metabolism.kit.compose.json",
);
const raw = fs.readFileSync(root, "utf8");
const doc = JSON.parse(raw);

function stripFrozenRootFromCheckpoints(head) {
  const items = head?.checkpoints?.items;
  if (!Array.isArray(items)) return;
  for (const cp of items) {
    if (cp && typeof cp === "object") delete cp.frozenRoot;
  }
}

stripFrozenRootFromCheckpoints(doc.wip);
delete doc.wip.drafts;

const w = doc.wip;
const theKit = {
  id: w.id,
  hash: STUB,
  savedChanges: { hash: STUB, items: [] },
  unsavedChanges: { hash: STUB, items: [] },
};

doc.wip = {
  id: w.id,
  hash: w.hash,
  authors: w.authors,
  initialKit: w.initialKit,
  checkpoints: w.checkpoints,
  theKit,
  alternatives: w.alternatives,
};

function secondaryGraphHead(kitId) {
  return {
    id: kitId,
    hash: STUB,
    authors: { hash: STUB, items: [] },
    initialKit: {
      hash: STUB,
      name: "",
      types: { hash: STUB, items: [] },
      designs: { hash: STUB, items: [] },
    },
    checkpoints: { hash: STUB, items: [] },
    theKit: {
      id: kitId,
      hash: STUB,
      savedChanges: { hash: STUB, items: [] },
      unsavedChanges: { hash: STUB, items: [] },
    },
    alternatives: { hash: STUB, items: [] },
  };
}

doc.authoritative = secondaryGraphHead(w.id);
doc.stage = secondaryGraphHead(w.id);

fs.writeFileSync(root, JSON.stringify(doc));
