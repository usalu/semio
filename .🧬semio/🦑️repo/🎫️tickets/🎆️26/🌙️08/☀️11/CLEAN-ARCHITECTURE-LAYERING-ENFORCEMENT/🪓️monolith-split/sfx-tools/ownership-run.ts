import { testCompositionOwnership } from "/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️ownership/🟦️.ts";
try { testCompositionOwnership(); console.log("OWNERSHIP PASS"); } catch (error) { console.log("OWNERSHIP FAIL", String((error as Error).message).slice(0, 600)); process.exit(1); }
