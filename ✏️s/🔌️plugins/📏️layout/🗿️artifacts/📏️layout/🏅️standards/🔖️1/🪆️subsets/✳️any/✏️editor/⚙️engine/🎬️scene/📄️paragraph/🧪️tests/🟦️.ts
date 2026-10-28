/** 🧪️ Shared Layout flow agrees with original source offsets and independent selected fonts. */
import {test,expect} from "bun:test";
import {run,repositoryRoot} from "./🧵️flow/🔮️oracle.mjs";
test("shared story flow and address cache",async()=>{const result=await run(repositoryRoot());expect(result.flow.runs).toBe(18);expect(result.flow.cancelled).toBe(6);expect(result.addressed.requests).toBe(60);expect(result.addressed.analyses).toBe(18);expect(result.labels.runs).toBe(15);expect(result.labels.cancelled).toBe(5);});
