const R = "/Users/ueli/Documents/semio";
const parent = `${R}/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/\u{1F7E6}️.ts`;
const scenario = `${R}/✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/\u{1F7E6}️.ts`;
const harness = await import(parent);
console.log("parent exports", Object.keys(harness));
const puzzle = await import(scenario);
console.log("scenario exports", Object.keys(puzzle));
const s = puzzle.puzzle2dTimeTravelScenario;
console.log(JSON.stringify({ variant: s.variant, serves: s.defaultServes, order: s.order.length, steps: Object.keys(s.steps).length, hooks: ["prepare", "beforeStep", "afterSteps", "diagnostics", "exploreInventory", "context"].map((k) => typeof s[k]) }));
