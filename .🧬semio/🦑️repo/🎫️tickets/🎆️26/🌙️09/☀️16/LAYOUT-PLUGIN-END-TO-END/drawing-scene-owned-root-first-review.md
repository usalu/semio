Root independently checked56 current inputs and readactual first test/strict logs; no source fix approved. Read exact failure and test ordering. Existing three release underfill probes occur after that release was already fully resumed; pure test-order correction authorized with same asserts and source/stock/data unchanged, exact inverse required.

{
  "changes": [],
  "logs": [
    {
      "kind": "test",
      "bytes": 6587,
      "sha256": "15bf1fcb375224e935773d23437318df4de5986fb55ca8eb93560d18bae9eb5c",
      "tail": "ases absent\");for(const phase of row.phases){const input=source(7,true),before=independentProjection(input.plan),w=bank(),job=new ScenePlanCopyJob(input.plan,{maxWork:policy.limits.maxWork});let reached=false;for(let at=0;at<policy.maxTurns;at++){const turn=w.run(job,work=>job.advance(1,work));if(turn.result.phase===phase){reached=true;break;}if(turn.result.done)break;}expect(reached).toBe(true);job.cancel();expect(()=>job.result()).toThrow();const moved=job.intoRetirement();expect(moved.output).toBeNull();close(moved.job,w);expect(independentProjection(input.plan)).toBe(before);conserved(w);}});\n48 | test(\"actual owned scene mutations refuse repeated underfilled grants and preserve same-bank resume\",()=>{const input=source(7,true),before=projection(input.plan),w=bank(),job=new ScenePlanCopyJob(input.plan,{maxWork:policy.limits.maxWork});for(const denied of [{maximumItems:0},{maximumDepth:0}]){const turn=w.run(job,work=>job.advance(1,work),1,denied);expect(turn.result.work).toBe(0);expect(turn.cpu).toBe(0);expect(turn.work.receipt).toEqual(zero);}const found=new Set<string>();for(let at=0;at<policy.maxTurns&&!job.resultReady();at++){const quote=job.nextWorkDemand();for(const[axis,key,amount]of[[\"copy\",\"maximumCopyBytes\",quote.copyBytes],[\"capacity\",\"maximumCapacityBytes\",quote.capacityBytes],[\"depth\",\"maximumDepth\",quote.depth]]as const){if(!amount||found.has(axis))continue;for(let repeat=0;repeat<3;repeat++){const denied=w.run(job,work=>job.advance(1,work),1,{[key]:amount-1});expect(denied.cpu).toBe(0);expect(denied.work.receipt).toEqual(zero);}found.add(axis);}w.run(job,work=>job.advance(1,work),1,{maxim\n\nerror: expect(received).toBe(expected)\n\nExpected: true\nReceived: false\n\n      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🗺️plan-model/🎟️owned/🧪️tests/owned.test.ts:48:2034)\n(fail) actual owned scene mutations refuse repeated underfilled grants and preserve same-bank resume [6.01ms]\n(pass) a real final admission posting failure preserves its own body CPU and every partial scene owner [0.48ms]\n\n 5 pass\n 1 fail\n 30704 expect() calls\nRan 6 tests across 1 file. [163.00ms]\n"
    },
    {
      "kind": "strict",
      "bytes": 132,
      "sha256": "0cf950d781c62ef5edf280ad364bb30da1f17d08f6bd6bf6f2be8ceec41d606d",
      "tail": "[tools] bun waits for machine capacity.\n[capacity] admitted command: 1 job(s), 2048 MB reserved (21050 MB available, 4096 MB floor)\n"
    }
  ]
}
