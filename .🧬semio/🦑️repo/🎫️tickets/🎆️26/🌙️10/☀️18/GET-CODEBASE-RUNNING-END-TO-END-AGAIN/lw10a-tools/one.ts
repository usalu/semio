const r = await Bun.build({ entrypoints: [process.argv[2]!], target: "bun", throw: false, packages: "external" });
for (const l of r.logs.slice(0, 3)) console.log(String(l.message), l.position?.file?.split("/").slice(-3).join("/"), l.position?.line, l.position?.column, JSON.stringify(l.position?.lineText?.slice(Math.max(0,(l.position?.column??0)-120),(l.position?.column??0)+80)));
