TL="⚡️caching/🚀️bootstrap/🛠️tools/"
def plan(p,L):
    t=L+TL+"🧪️tests/🟦️.ts"
    p.sub(t,'expect(source).toContain("if (present) return withResourceLeases(");expect(source.indexOf("if (present) return withResourceLeases(")).toBeLessThan(source.indexOf("return publisher(async()=>{"));','expect(source).toContain("if (present) return ready(await withResourceLeases(");expect(source.indexOf("if (present) return ready(await withResourceLeases(")).toBeLessThan(source.indexOf("return publisher(async()=>{"));')
