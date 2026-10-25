F="🧰️framework/"
P=F+"🔨️modules/🏃️process/"
PK=P+"📦️packages/🟦️typescript/"
L=F+"🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
DOMAINS={
 "leases":[P+"🔒️leases/",PK+"🔒️leases/"],
 "capacity":[P+"🏗️build/🎟️capacity/",P+"🏗️build/🌡️host-capacity/",PK+"🏗️build/🎟️capacity/",B+"🌡️host-capacity/"],
 "store-custody":[P+"📦️artifacts/🏗️native-build/",P+"📦️artifacts/💾️cache/",PK+"📦️artifacts/🏗️native-build/",PK+"📦️artifacts/💾️cache/",L+"⚡️caching/💾️storage/",B+"🕸️graph/🕰️generations/"],
 "process-supervision":[P+"🪓️termination/",P+"💾️state/",PK+"💾️state/",P+"🪪️agent-sessions/",P+"🧭️routing/📥️invocation/",PK+"🧭️routing/📥️invocation/"],
 "retry-observation":[F+"🔨️modules/📁️filesystem/🧾️observation/🔁️churn/"],
 "progress-reporting":[P+"🏗️build/🗣️language/",PK+"🏗️build/🗣️language/",P+"📖️texts/",B+"📖️texts/",L+"⚡️caching/📖️texts/"],
}
def domain(p):
    for k,pre in DOMAINS.items():
        if any(p.startswith(x) for x in pre): return k
    return None
