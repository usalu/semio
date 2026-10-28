/** 🧪️ Original interactive paragraph publication agrees with independent OpenType, SVG pixels, and authored defaults. */
import {test,expect} from "bun:test";
import {run,repositoryRoot} from "./🔮️oracle.mjs";
test("interactive outline publication, styles and original cancellation",async()=>{const result=await run(repositoryRoot());expect(result.styles).toBe(4);expect(result.pixels).toBe(24);expect(result.maxPixelError).toBe(0);expect(result.cancellations).toBe(12);expect(result.refusals).toBe(8);});
