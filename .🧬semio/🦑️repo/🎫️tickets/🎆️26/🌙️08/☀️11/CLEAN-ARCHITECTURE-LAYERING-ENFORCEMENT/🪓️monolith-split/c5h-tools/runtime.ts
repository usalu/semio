import * as leaf from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
const names = Object.keys(leaf).filter((name) => name.startsWith("parse"));
console.log("exported parsers", names.length);
console.log(JSON.stringify(leaf.parseProgramStakeholdersPatchEntry({ id: "a", patch: { x: 1 } })));
try { leaf.parseProgramStakeholdersPatchEntry({ id: "a", patch: {}, extra: 1 }); console.log("accepted extra?!"); } catch (error) { console.log("refused:", (error as Error).message); }
try { leaf.parseProgramRisksPatchEntry({ id: 1, patch: {} }); console.log("accepted?!"); } catch (error) { console.log("refused:", (error as Error).message); }
