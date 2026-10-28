/** 🧪️ Third-party JSON Patch is isolated behind an owned semantic test interface. */
import jsonPatch from 'fast-json-patch';
export function projectPatch(before, patch) { return jsonPatch.applyPatch(JSON.parse(JSON.stringify(before)), patch, true, false).newDocument; }
export function independentDifference(before, after) { return jsonPatch.compare(before, after); }
