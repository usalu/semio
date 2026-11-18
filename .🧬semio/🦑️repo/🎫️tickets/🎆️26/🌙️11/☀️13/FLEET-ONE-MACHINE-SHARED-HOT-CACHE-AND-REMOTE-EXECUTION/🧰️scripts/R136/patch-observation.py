def patch(t):
    old="const youngest=Math.max(0,...rows.map(({kind,fact})=>{const source=kind===\"file\"?fact:kind===\"analysis\"?fact.source:undefined;return source?Math.max(source.mtimeMs??0,source.ctimeMs??0):0;}));"
    new="const youngest=rows.reduce((most,{kind,fact})=>{const source=kind===\"file\"?fact:kind===\"analysis\"?fact.source:undefined;return source?Math.max(most,source.mtimeMs??0,source.ctimeMs??0):most;},0);"
    assert t.count(old)==1
    return t.replace(old,new)
