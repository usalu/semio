p="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🧪️tests/🟦️.ts"
s=open(p,encoding='utf8').read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(a[:70],s.count(a))
    s=s.replace(a,b)
rep('publishedAtMs:1,cache:{endpoints:["http://10.0.0.5:6090"]','publishedAtMs:1,publisher:{directory:"/var/lib/semio-remote-cache",port:6090,host:"h1"},cache:{endpoints:["http://10.0.0.5:6090"]')
rep('({storeRoot,directory:cache,group:"g1",instance:"semio",endpoints,now})','({storeRoot,directory:cache,group:"g1",instance:"semio",endpoints,port:6090,host:"h1",now})')
rep('{...sample,generation:0},','{...sample,generation:0},{...sample,publisher:undefined},{...sample,publisher:{directory:"",port:6090,host:"h"}},')
# plan test
a=s.index('test("the server plan publishes only')
b=s.index(' const interfaces=',a)
s=s[:a]+'''test("the server plan publishes only for the group server with the production identity or an explicit scratch store, and refuses the real store otherwise",async()=>{
 const root=directory("plan"),real=directory("real"),scratch=directory("scratch"),production={directory:"/var/lib/semio-remote-cache",port:6090},other={directory:directory("data"),port:6090};
 expect(await fleetServerPlanV1(root,{},"main",production),"no machine.json").toBeUndefined();
 machine(root,real,"g1-m03");expect(await fleetServerPlanV1(root,{},"main",production),"a worker never publishes").toBeUndefined();
 machine(root,real,"g1-server");
 expect(await fleetServerPlanV1(root,{SEMIO_REMOTE_CACHE_SERVER_FLEET:"off"},"main",production)).toBeUndefined();
 expect(await fleetServerPlanV1(root,{},"main",other),"a scratch server publishes nothing").toBeUndefined();
 expect(await fleetServerPlanV1(root,{},"main",{directory:"/var/lib/semio-remote-cache",port:6191}),"the right directory on another port").toBeUndefined();
 expect(await fleetServerPlanV1(root,{},"main",{directory:other.directory,port:undefined}),"the provisioned port alone is not enough").toBeUndefined();
 await expect(fleetServerPlanV1(root,{SEMIO_REMOTE_CACHE_SERVER_FLEET:"publish"},"main",other)).rejects.toThrow("refused");
 await expect(fleetServerPlanV1(root,{SEMIO_REMOTE_CACHE_SERVER_FLEET_STORE:real},"main",other),"the real store without the production identity").rejects.toThrow("real store");
 expect(await fleetServerPlanV1(root,{SEMIO_REMOTE_CACHE_SERVER_FLEET_STORE:scratch},"main",other)).toMatchObject({storeRoot:scratch,production:false});
 expect(await fleetServerPlanV1(root,{SEMIO_REMOTE_CACHE_INSTANCE:"mine",SEMIO_FLEET_CACHE_ENDPOINT:"http://10.1.1.1:6090"},"main",production)).toMatchObject({storeRoot:real,group:"g1",instance:"mine",given:["http://10.1.1.1:6090"],production:true});
 expect((await fleetServerPlanV1(root,{SEMIO_REMOTE_CACHE_SERVER_FLEET:"publish"},"main",production))?.storeRoot).toBe(real);
 expect((await fleetServerPlanV1(root,{},"main",production))?.instance).toBe("main");
'''+s[b:]
# serve test: separate real store
rep('const root=directory("server-root"),store=directory("store"),cache=directory("cache");machine(root,store,"g1-server");\n const written','const root=directory("server-root"),real=directory("real-store"),store=directory("store"),cache=directory("cache");machine(root,real,"g1-server");\n const written')
rep('environment:{SEMIO_FLEET_CACHE_ENDPOINT:"http://192.0.2.9:6090"},signal','environment:{SEMIO_FLEET_CACHE_ENDPOINT:"http://192.0.2.9:6090",SEMIO_REMOTE_CACHE_SERVER_FLEET_STORE:store},signal')
rep(' expect(published).toMatchObject({group:"g1",generation:1,cache:{instance:"main"}});',' expect(published).toMatchObject({group:"g1",generation:1,cache:{instance:"main"},publisher:{directory:cache,port:Number(new URL(url).port)}});expect(existsSync(fleetCacheFactsPathV1(real)),"the real store stays untouched").toBe(false);')
s+='''
test("a scratch server leaves existing facts byte-identical, and a publisher never lowers the generation or replaces recent facts of another identity",async()=>{
 const real=directory("real"),root=directory("root"),cache=directory("cache"),theirs=directory("theirs"),other=directory("other");machine(root,real,"g1-server");
 const production=await publishFleetCacheFactsV1({storeRoot:real,directory:theirs,group:"g1",instance:"semio",endpoints:["http://10.0.0.5:6090"],port:6090,host:"real-server"});
 const before=readFileSync(fleetCacheFactsPathV1(real));
 const controller=new AbortController(),lines:string[]=[],context=(environment:Record<string,string>):RemoteCacheCommandContextV1=>({root,environment,signal:controller.signal,progress:async()=>{},write:line=>lines.push(line),tools:async()=>[]});
 const scratch=serveRemoteCacheV1(["--host","127.0.0.1","--port","0","--directory",cache,"--lang","en"],context({}));
 for(let wait=0;wait<100&&!lines.some(line=>line.includes("listening"));wait++)await Bun.sleep(50);
 await Bun.sleep(300);controller.abort();await scratch;
 expect(readFileSync(fleetCacheFactsPathV1(real)).equals(before),"a scratch serve leaves the real facts byte-identical").toBe(true);expect(existsSync(join(cache,"fleet")),"and mints no credential").toBe(false);
 await expect(serveRemoteCacheV1(["--host","127.0.0.1","--port","0","--directory",cache],context({SEMIO_REMOTE_CACHE_SERVER_FLEET:"publish"}))).rejects.toThrow("refused");
 await expect(serveRemoteCacheV1(["--host","127.0.0.1","--port","0","--directory",cache],context({SEMIO_REMOTE_CACHE_SERVER_FLEET_STORE:real}))).rejects.toThrow("real store");
 expect(readFileSync(fleetCacheFactsPathV1(real)).equals(before)).toBe(true);expect(existsSync(join(cache,"fleet"))).toBe(false);
 const attempt=(directory:string,now:number,port=6191)=>publishFleetCacheFactsV1({storeRoot:real,directory,group:"g1",instance:"semio",endpoints:["http://10.0.0.5:6191"],port,host:"scratch",now});
 await expect(attempt(other,production.facts.publishedAtMs+1000)).rejects.toThrow("recent facts of another publisher");
 await expect(attempt(other,production.facts.publishedAtMs+11*60000),"a stale foreign publisher without a higher generation").rejects.toThrow("does not exceed");
 expect(readFileSync(fleetCacheFactsPathV1(real)).equals(before),"every refusal leaves the file byte-identical").toBe(true);
 const lowerStore=directory("lower"),high=directory("high"),low=directory("low");
 await rotateFleetCredentialV1(high);await rotateFleetCredentialV1(high);
 const mine=await publishFleetCacheFactsV1({storeRoot:lowerStore,directory:high,group:"g1",instance:"semio",endpoints:["http://10.0.0.5:6090"],port:6090,host:"h",now:1000});expect(mine.facts.generation).toBe(3);
 await expect(publishFleetCacheFactsV1({storeRoot:lowerStore,directory:low,group:"g1",instance:"semio",endpoints:["http://10.0.0.5:6091"],port:6091,host:"h",now:1000+11*60000}),"another identity with a lower generation").rejects.toThrow("does not exceed");
 const raised=directory("raised"),own=directory("own");
 await publishFleetCacheFactsV1({storeRoot:raised,directory:own,group:"g1",instance:"semio",endpoints:["http://10.0.0.5:6090"],port:6090,host:"h",now:1});
 const text=JSON.parse(readFileSync(fleetCacheFactsPathV1(raised),"utf8"));text.generation=7;writeFileSync(fleetCacheFactsPathV1(raised),JSON.stringify(text));
 const again=await publishFleetCacheFactsV1({storeRoot:raised,directory:own,group:"g1",instance:"semio",endpoints:["http://10.0.0.5:6090"],port:6090,host:"h",now:2});expect(again.facts.generation,"the same identity raises its own credential above the published generation").toBe(8);
});

test("a refresh ignores facts whose endpoints all fail and keeps the previous good local copy",async()=>{
 const store=directory("store"),cache=directory("cache"),dead=directory("dead"),root=directory("worker");machine(root,store);
 const server=await serving(cache);
 await publishFleetCacheFactsV1(publication(store,cache,[server.url]));
 const now=Date.now(),good=await refreshFleetCacheFactsV1({root,probeMilliseconds:300,now});expect(good.state).toBe("updated");expect(good.copy?.selectedEndpoint).toBe(server.url);
 rmSync(fleetCacheFactsPathV1(store));
 await publishFleetCacheFactsV1({storeRoot:store,directory:dead,group:"g1",instance:"semio",endpoints:["http://127.0.0.1:1"],port:6191,host:"scratch"});
 const ignored=await refreshFleetCacheFactsV1({root,force:true,probeMilliseconds:300});
 expect(ignored.state).toBe("stale");expect(ignored.error).toContain("previous good copy");expect(ignored.copy?.facts.cache.endpoints).toEqual([server.url]);
 expect(resolveRemoteCacheV1({SEMIO_WORKSPACE_ROOT:root},{root})?.contract.endpoint,"the dead endpoint is not adopted").toBe(server.url);
});
'''
open(p,'w',encoding='utf8').write(s)
