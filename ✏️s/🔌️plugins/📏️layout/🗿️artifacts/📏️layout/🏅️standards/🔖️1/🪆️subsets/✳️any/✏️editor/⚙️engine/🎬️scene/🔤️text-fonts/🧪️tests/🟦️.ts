/** 🧪️ Headless built-in resources retain exact fonts and strict Unicode fallback. */
import {test,expect} from "bun:test";
import {run,repositoryRoot} from "./🔮️oracle.mjs";
test("headless font resources agree with independent coverage",()=>{const result=run(repositoryRoot());expect(result.fonts).toBe(16);expect(result.unicodeGlyphs).toBe(7);expect(result.refusals).toBe(4);});
