const s=JSON.parse(await Bun.file(process.argv[2]!).text());
for(const f of s.files){const t=Buffer.from(f.base64,"base64").toString("utf8");if(/createRequire|\brequire\(\s*["'`]\.|\brequire\(/.test(t)){for(const m of t.matchAll(/.{0,60}(createRequire\([^)]*\)|\brequire\()\s*[^)]{0,80}\)?.{0,30}/g))console.log(f.path.slice(-90),"|",m[0].replace(/\n/g," "));}}
