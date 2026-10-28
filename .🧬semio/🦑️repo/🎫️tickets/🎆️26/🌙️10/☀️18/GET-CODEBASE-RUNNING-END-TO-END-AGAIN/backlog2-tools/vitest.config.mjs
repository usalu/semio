import {defineConfig} from "vitest/config";
const root=process.env.COURSE_ROOT;
export default defineConfig({root,cacheDir:(process.env.TMPDIR||"/tmp")+"/b2-vitest-cache",test:{root,name:"b2-course",environment:"node",include:(process.env.COURSE_INCLUDE||"🧪️tests/🏗️proctor-build/🟦️.ts").split(",")}});
