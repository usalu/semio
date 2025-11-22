// #region Header

// debug-kit-equality.mjs

// Diagnostic script to compare original and imported kits

// #endregion

import { MetabolismKit } from "@semio/assets";
import { exportKit, importKit } from "./semio.ts";

const originalKit = MetabolismKit;
const files = new Map();

console.log("[DEBUG] Starting export...");
const zipBlob = await exportKit(originalKit, files);
console.log(`[DEBUG] Export complete: ${zipBlob.size} bytes`);

const url = URL.createObjectURL(zipBlob);
console.log("[DEBUG] Starting import...");
const { kit: importedKit, files: importedFiles } = await importKit(url);
console.log("[DEBUG] Import complete");
URL.revokeObjectURL(url);

// Deep comparison
const compare = (path, a, b) => {
  if (a === b) return true;
  
  if (typeof a !== typeof b) {
    console.log(`[DIFF] ${path}: type mismatch - ${typeof a} vs ${typeof b}`);
    return false;
  }
  
  if (a === null || b === null || a === undefined || b === undefined) {
    if (a !== b) {
      console.log(`[DIFF] ${path}: null/undefined - ${a} vs ${b}`);
      return false;
    }
    return true;
  }
  
  if (typeof a === 'object') {
    if (Array.isArray(a) !== Array.isArray(b)) {
      console.log(`[DIFF] ${path}: array vs object`);
      return false;
    }
    
    if (Array.isArray(a)) {
      if (a.length !== b.length) {
        console.log(`[DIFF] ${path}: array length - ${a.length} vs ${b.length}`);
        return false;
      }
      for (let i = 0; i < a.length; i++) {
        compare(`${path}[${i}]`, a[i], b[i]);
      }
      return true;
    }
    
    const keysA = Object.keys(a).sort();
    const keysB = Object.keys(b).sort();
    
    const allKeys = new Set([...keysA, ...keysB]);
    for (const key of allKeys) {
      if (!(key in a)) {
        console.log(`[DIFF] ${path}.${key}: missing in original`);
        continue;
      }
      if (!(key in b)) {
        console.log(`[DIFF] ${path}.${key}: missing in imported`);
        continue;
      }
      compare(`${path}.${key}`, a[key], b[key]);
    }
    return true;
  }
  
  if (a !== b) {
    console.log(`[DIFF] ${path}: value - ${JSON.stringify(a)} vs ${JSON.stringify(b)}`);
    return false;
  }
  
  return true;
};

console.log("\n[DEBUG] Comparing kits...\n");
compare('kit', originalKit, importedKit);
console.log("\n[DEBUG] Comparison complete");
