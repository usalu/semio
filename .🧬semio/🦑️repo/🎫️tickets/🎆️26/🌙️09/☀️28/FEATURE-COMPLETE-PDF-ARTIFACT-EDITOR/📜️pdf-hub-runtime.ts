/** 🏘️ Runs the PDF Check In fixture through the repository's real local Hub and Studio owners. */
import { mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd(), output = resolve(process.argv[2] ?? join(import.meta.dir,"🗑️generated","authenticated-pdf"));
mkdirSync(output,{recursive:true});
const load = (path:string) => import(pathToFileURL(resolve(root,path)).href);
const repo = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const processOwner = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript";
const hub = "🌎️hub", collaboration = hub+"/🧪️tests/🤝️dev-collaboration";
const provenance = hub+"/🏗️bootstrap/🧾️provenance";
const [{createNativeBuildCallerCapabilitiesV1},{issueRepositoryScriptProvidersV1},routing,{scriptCompilerAdmissionControlV1},{runOwnedCommand}] = await Promise.all([
  load(repo+"/🏗️build/📥️caller/🟦️.ts"),load(repo+"/🏃️process/🔌️providers/🟦️.ts"),load(processOwner+"/🧭️routing/🟦️.ts"),
  load(processOwner+"/🏗️build/🎟️capacity/🟦️.ts"),load(processOwner+"/🎛️execution/🟦️.ts"),
]);
process.env.SEMIO_BUILD_MODE = "prototype";
issueRepositoryScriptProvidersV1(process.env);
const caller = createNativeBuildCallerCapabilitiesV1(root,["run","os-hub:dev-collaboration","--exclude-dependencies","--configuration=prototype"],"nx",process.env);
const original = routing.createScriptProcessEnvelope({version:1,owner:"pdf:authenticated-browser",maximumElapsedMilliseconds:0},caller,Date.now());
Object.assign(process.env,routing.scriptProcessEnvironment(original,process.env));

await routing.receiveScriptProcessInvocation(process.env,async invocation=>{
  const phase = scriptCompilerAdmissionControlV1(invocation,"pdf:authenticated-browser",0);
  const [host,bundles,publication,selection,tools,scope,credentials,defaults] = await Promise.all([
    load(hub+"/🚀️local-bootstrap/🎛️host/🟦️.ts"),
    load(provenance+"/🚚️bootstrap-publication/🧱️materialized-bundle/🟦️.ts"),
    load(provenance+"/🚚️bootstrap-publication/🟦️.ts"),
    load(provenance+"/🧭️bootstrap-selection/🟦️.ts"),
    load(hub+"/🚀️local-bootstrap/🏃️execution/🟦️.ts"),
    load(collaboration+"/🧭️scope-and-dev-servers/🟦️.ts"),
    load(collaboration+"/🏠️hub-start-and-credentials/🟦️.ts"),
    load(collaboration+"/⚙️constants-relay-and-recorder/🟦️.ts"),
  ]);
  await tools.withBuiltLocalHubHostV1(invocation,phase,(retained,context)=>
    bundles.withMaterializedTrustedCatalogBundleV1(retained,context.control,selection.trustedBootstrapSelectPackages("stdio,gis,stdio-pdf"),async(receipt,control)=>{
      await publication.validateAndPublishTrustedStdioGisCandidate(retained,control,receipt);
      await tools.withLocalHubHostTaskV1(retained,control,actual=>{
        const owner = {host:retained,context:actual,servers:new Set(),ledger:new host.LocalHubHostLedgerV1(),live:true,serverBirths:0};
        return scope.collabScopesV1.run(owner,async()=>{
          await credentials.collabProvisionCredentials();
          const ports = new Set<number>();
          const hubPort = credentials.collabScanPort("S_COLLAB_HUB_PORT",ports);
          const firstPort = credentials.collabScanPort("S_COLLAB_USER1_PORT",ports);
          const peerPort = credentials.collabScanPort("S_COLLAB_USER2_PORT",ports);
          const local = (port:number) => "http://127.0.0.1:"+port;
          const service = await credentials.collabStartHub(retained,hubPort,"boot");
          const users: string[] = [defaults.COLLAB_E2E_USER1_EMAIL,defaults.COLLAB_E2E_USER2_EMAIL];
          try {
            mkdirSync(join(scope.collabOutDir(),"user1"),{recursive:true});
            mkdirSync(join(scope.collabOutDir(),"user2"),{recursive:true});
            scope.collabActivateShellRuntime();
            await Promise.all([firstPort,peerPort].map((port,index)=>scope.collabStartUserDevServer({port,hubUrl:local(hubPort),user:users[index]})));
            const env = routing.scriptInvocationEnvironment(invocation,{...process.env,
              SEMIO_TWO_HUMAN_USER1_EMAIL:users[0],SEMIO_TWO_HUMAN_USER2_EMAIL:users[1],
              SEMIO_TWO_HUMAN_USER1_PASSWORD:defaults.COLLAB_E2E_USER1_PASSWORD,SEMIO_TWO_HUMAN_USER2_PASSWORD:defaults.COLLAB_E2E_USER2_PASSWORD,
            });
            const png = resolve(process.argv[3] ?? join(import.meta.dir,"🗑️generated","large-picked","large-picked.png"));
            await runOwnedCommand(process.execPath,[join(import.meta.dir,process.env.PDF_HUB_SCRIPT ?? "📜️pdf-hub-image-check-in.ts"),local(hubPort),local(firstPort),output,png,local(peerPort)],root,
              "pdf:authenticated-check-in",0,{env,signal:actual.control.signal,deadline:actual.control,stdout:"inherit"});
          } finally {
            owner.live = false;
            try { await Promise.all([...owner.servers].map(scope.finishCollabDevServerV1)); }
            finally { await service.kill(); }
          }
        });
      });
    }));
});
