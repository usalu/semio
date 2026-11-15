const [slug, scale] = process.argv.slice(2) as [string, string];
const mod: any = await import(`./author-01-${slug}.ts`);
const a = mod.build(Number(scale));
console.log(a.outside().filter(([, w]: any) => w > a.feature * 0.9).map(([i, w]: any) => `${i}:${w.toFixed(3)}`).join(" "));
console.log("strays", a.strays().join(" "));
