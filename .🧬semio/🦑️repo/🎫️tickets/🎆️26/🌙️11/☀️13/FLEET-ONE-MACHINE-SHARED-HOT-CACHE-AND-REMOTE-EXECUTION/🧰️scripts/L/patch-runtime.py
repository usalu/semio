R="/Users/Shared/semio/🧰️framework/🔨️modules/⚙️runtime/"
def edit(path,pairs):
    s=open(R+path,encoding='utf8').read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:70])
        s=s.replace(old,new)
    open(R+path,'w',encoding='utf8').write(s)
edit('📋️contract/🟦️.ts',[('readonly tls?: Readonly<{ certificate: string; key: string }>; readonly fetch: (request: Request) => Response | Promise<Response> }','readonly tls?: Readonly<{ certificate: string; key: string }>; readonly fetch: (request: Request, peer?: string) => Response | Promise<Response> }')])
edit('🍞️bun/🧩️tooling/🟦️.ts',[('}), fetch: options.fetch });','}), fetch: (request, server) => options.fetch(request, server.requestIP(request)?.address) });')])
edit('🪶️portable/🌍️serve/🟦️.ts',[('await options.fetch(requestOf(incoming, hostname, boundPort, options.maximumRequestBodyBytes, options.tls === undefined ? "http" : "https"))','await options.fetch(requestOf(incoming, hostname, boundPort, options.maximumRequestBodyBytes, options.tls === undefined ? "http" : "https"), incoming.socket.remoteAddress)')])
print("runtime ok")
