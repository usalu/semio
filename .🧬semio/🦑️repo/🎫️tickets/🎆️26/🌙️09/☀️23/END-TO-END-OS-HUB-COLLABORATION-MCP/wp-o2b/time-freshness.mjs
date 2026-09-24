
const t0 = Date.now();
const mark = (l) => console.log(`+${Date.now()-t0}ms ${l}`);
mark("import-start");
const freshness = await import("/Users/ueli/Documents/semio/.tmp-ticket/wp-o2b/links/freshness.ts");
mark("freshness-imported");
const act = await import("/Users/ueli/Documents/semio/.tmp-ticket/wp-o2b/links/activation.ts");
mark("activation-imported");
const osdev = "/Users/ueli/Documents/semio/.tmp-ticket/wp-o2b/links/os-dev-pkg";
const runtime = act.developmentRuntimeRoot(osdev, "s", "dev", "react");
mark("runtime="+runtime);
const receiptRoot = runtime + "/activation";
const receipt = act.readActivationReceipt(receiptRoot);
mark("receipt-plugins="+receipt.plugins.length);
freshness.reportServeStagedModuleFreshness("s","react","dev", runtime, receipt);
mark("freshness-done");
