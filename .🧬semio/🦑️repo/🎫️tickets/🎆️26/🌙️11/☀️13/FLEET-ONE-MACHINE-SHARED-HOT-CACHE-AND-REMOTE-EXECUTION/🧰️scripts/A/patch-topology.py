M="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/"
def edit(path,pairs):
    s=open(path,encoding='utf8').read()
    for a,b in pairs:
        assert s.count(a)==1,(a[:70],s.count(a))
        s=s.replace(a,b)
    open(path,'w',encoding='utf8').write(s)
edit(M+"🟦️.ts",[
 ('`EnvironmentFile=${TOKENS}`,"Environment=SEMIO_LANGUAGE=en",','`EnvironmentFile=${TOKENS}`,"Environment=SEMIO_LANGUAGE=en",`Environment=${FLEET_PUBLISH}`,'),
 ('`set -a; . ${TOKENS}; set +a; exec ${REPO} remote-cache serve','`set -a; . ${TOKENS}; set +a; export ${FLEET_PUBLISH}; exec ${REPO} remote-cache serve'),
 ("Set-Item -Path ('Env:'+$n) -Value $v }; & repo remote-cache serve","Set-Item -Path ('Env:'+$n) -Value $v }; $env:SEMIO_REMOTE_CACHE_SERVER_FLEET='publish'; & repo remote-cache serve"),
 ('MACOS_CACHE_LABEL="tech.semio.fleet.cache",','MACOS_CACHE_LABEL="tech.semio.fleet.cache",\n FLEET_PUBLISH="SEMIO_REMOTE_CACHE_SERVER_FLEET=publish",'),
])
edit(M+"🧪️tests/🟦️.ts",[
 ('expect(parts["cache-unit"]).toContain("ExecStart=/usr/local/bin/repo remote-cache serve --host 0.0.0.0 --port 6090','expect(parts["cache-unit"]).toContain("Environment=SEMIO_REMOTE_CACHE_SERVER_FLEET=publish");expect(parts["cache-unit"]).toContain("ExecStart=/usr/local/bin/repo remote-cache serve --host 0.0.0.0 --port 6090'),
 ('set -a; . /etc/semio/remote-cache.env; set +a; exec /usr/local/bin/repo remote-cache serve --host 0.0.0.0 --port 7000','set -a; . /etc/semio/remote-cache.env; set +a; export SEMIO_REMOTE_CACHE_SERVER_FLEET=publish; exec /usr/local/bin/repo remote-cache serve --host 0.0.0.0 --port 7000'),
 ('expect(nfs["cache-unit"]).toContain("remote-cache serve --host 0.0.0.0 --port 443','expect(nfs["cache-unit"]).toContain("$env:SEMIO_REMOTE_CACHE_SERVER_FLEET=\'publish\'; & repo remote-cache serve");expect(nfs["cache-unit"]).toContain("remote-cache serve --host 0.0.0.0 --port 443'),
])
print("ok")
