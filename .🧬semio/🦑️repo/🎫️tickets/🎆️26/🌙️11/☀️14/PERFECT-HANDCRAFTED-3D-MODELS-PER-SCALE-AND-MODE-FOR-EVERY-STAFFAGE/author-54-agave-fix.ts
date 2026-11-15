import { loftOf, openCtx, saveLists, superellipseSection, featureM, type Json } from "./kit-lib.ts";

const scale = Number(process.argv[2]);
const c = openCtx("vegetation/agave", scale);
const F = featureM(scale);
const st: Json[] = c.vol.structure;
const k = st.findIndex((s) => s.id === "member-root-plate");
const a = scale === 50 ? 0.08 : scale === 100 ? 0.09 : 0.1;
const r = Math.max(F * 0.8, 0.012);
st[k] = { ...loftOf({ id: "member-root-plate", bone: "root-plate", material: "agave-leaf", tone: 0.5 }, [superellipseSection([-a, Math.max(r + 0.01, 0.03), 0], r, r, 2), superellipseSection([0, Math.max(r + 0.01, 0.03), 0], 0.09, r, 2.4), superellipseSection([a, Math.max(r + 0.01, 0.03), 0], r, r, 2)]), member: "root-plate", part: "member" };
saveLists(c, { structure: st });
