import {tmpdir} from "node:os";
import {join} from "node:path";
import {defineConfig} from "vitest/config";

export default defineConfig({cacheDir:join(tmpdir(),"semio-vitest-leases-routing"),test:{include:["🟦️.ts"],environment:"node"}});
