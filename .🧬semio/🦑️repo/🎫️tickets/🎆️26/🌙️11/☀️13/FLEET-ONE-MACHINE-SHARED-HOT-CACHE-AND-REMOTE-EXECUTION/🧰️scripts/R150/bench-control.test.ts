import{test}from"bun:test";
import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import*as issuer from"@semio-tech/process/routing/invocation/process";
import*as leases from"@semio-tech/process/leases";
test("bench control",()=>withBuildTestInvocationV1(async original=>{
 const stop=new AbortController(),c=issuer.createIssuedResourceControlV1(original,"native:input-stage-source",300000,stop.signal),N=2000;
 let t=performance.now();for(let i=0;i<N;i++)await c.publish({resource:"native-artifact-input-stage:"+i,mode:"exclusive",elapsedMs:0});
 const pub=(performance.now()-t)/N;
 t=performance.now();for(let i=0;i<N;i++)await c.yieldContinuation();
 const yl=(performance.now()-t)/N;
 t=performance.now();for(let i=0;i<N*10;i++)leases.checkResourceLeaseControl(c);
 const chk=(performance.now()-t)/(N*10);
 console.log("[BENCH] "+JSON.stringify({publishMs:+pub.toFixed(4),yieldMs:+yl.toFixed(4),checkMs:+chk.toFixed(5)}));
}));
