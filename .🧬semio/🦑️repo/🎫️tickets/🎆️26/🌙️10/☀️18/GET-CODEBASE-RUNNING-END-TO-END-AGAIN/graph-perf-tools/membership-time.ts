// ⏱️ Times the real nxGraphSourceMembershipV2 (git inventory) against the live checkout with the 10 s deadline lifted; read-only.
const root=process.argv[2]??"/Users/ueli/Documents/semio",deadline=Number(process.env.DEADLINE_MS??"600000");
Bun.plugin({name:"membership-deadline",setup(b){b.onLoad({filter:/membership\/[^/]*\.ts$/},async args=>({contents:(await Bun.file(args.path).text()).replace("10_000)",`${deadline})`),loader:"ts"}));}});
const {nxGraphSourceMembershipV2}=await import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/📥️membership/🟦️.ts");
const t=performance.now();
const r=await nxGraphSourceMembershipV2(root);
console.log(JSON.stringify({files:r.files.length,digest:r.digest.slice(0,16),wallMs:Math.round(performance.now()-t)}));
