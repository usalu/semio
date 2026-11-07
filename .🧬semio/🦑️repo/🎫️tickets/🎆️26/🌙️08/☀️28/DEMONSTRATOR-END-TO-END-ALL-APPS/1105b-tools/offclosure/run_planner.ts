import {testDrawingPlanner} from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🟦️.ts";
import {plan,paint} from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/oracle.mjs";
testDrawingPlanner({plan:(row:any)=>plan(row),paint} as any);
console.log("planner corpus ok");
