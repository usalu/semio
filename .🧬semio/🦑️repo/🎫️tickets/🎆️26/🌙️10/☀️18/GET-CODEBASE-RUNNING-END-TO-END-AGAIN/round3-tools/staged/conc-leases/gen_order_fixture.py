import json,os
HERE=os.path.dirname(os.path.abspath(__file__))
OUT=os.path.join(HERE,"staged","🧰️framework","🔨️modules","🏃️process","🔒️leases","🧭️order","🧪️tests","🧫️fixtures","🔣️.json")
D="/lease"
policy={
 "version":1,
 "rules":{"outOfOrder":"enforce","holdAndWait":"enforce","selfDeadlock":"enforce","cycle":"enforce"},
 "holdAndWait":{"graceMilliseconds":1000,"pollMilliseconds":50},
 "cycle":{"intervalMilliseconds":50},
 "registry":{"flushMilliseconds":0,"maximumEntryBytes":65536,"maximumViolations":50},
 "fairness":{"queuedWithResourceLeases":True},
 "gate":{"releaseWhileWaiting":True,"pollInitialMilliseconds":5,"pollMaximumMilliseconds":40},
 "classes":[
  {"id":"outer","rank":1,"scope":"checkout","owner":"test","title":{"en":"Outer","de":"Außen"},"match":[{"kind":"prefix","value":"outer:"}]},
  {"id":"middle","rank":2,"scope":"project","owner":"test","title":{"en":"Middle","de":"Mitte"},"match":[{"kind":"prefix","value":"middle:"},{"kind":"exact","value":"middle-singleton"}]},
  {"id":"inner","rank":3,"scope":"path","owner":"test","title":{"en":"Inner","de":"Innen"},"match":[{"kind":"prefix","value":"inner:"},{"kind":"prefix","value":"inner:special:"}]},
  {"id":"free","rank":0,"scope":"session","owner":"test","title":{"en":"Free","de":"Frei"},"match":[{"kind":"prefix","value":"free:"}]}
 ],
 "allowed":[{"from":"inner","to":"middle","reason":{"en":"Declared exception.","de":"Erklärte Ausnahme."}}]
}
h=lambda r,m="exclusive",t=1,at=1000,d=D:{"directory":d,"resource":r,"mode":m,"atMs":at,"task":t}
q=lambda r,m="exclusive",nb=False,d=D,b=False:{"directory":d,"resource":r,"mode":m,"nonblocking":nb,"bounded":b}
fixture={
 "version":1,
 "policy":policy,
 "classification":[
  {"resource":"outer:a","class":"outer"},{"resource":"middle-singleton","class":"middle"},{"resource":"middle:x","class":"middle"},
  {"resource":"inner:special:z","class":"inner"},{"resource":"free:q","class":"free"},{"resource":"unknown","class":"unclassified"},{"resource":"middle-singleton-extra","class":"unclassified"}
 ],
 "order":[
  {"id":"inward-allowed","held":[h("outer:a")],"request":q("inner:b"),"expect":None},
  {"id":"outward-refused","held":[h("inner:b")],"request":q("outer:a"),"expect":{"kind":"out-of-order","heldClass":"inner","requestedClass":"outer"}},
  {"id":"declared-edge-exempt","held":[h("inner:b")],"request":q("middle:x"),"expect":None},
  {"id":"unclassified-never-constrained","held":[h("inner:b")],"request":q("unknown"),"expect":None},
  {"id":"free-rank-zero-never-constrained","held":[h("inner:b")],"request":q("free:q"),"expect":None},
  {"id":"try-lock-exempt","held":[h("inner:b")],"request":q("outer:a","exclusive",True),"expect":None},
  {"id":"same-rank-allowed","held":[h("middle:b")],"request":q("middle:a"),"expect":None},
  {"id":"self-deadlock-shared-then-exclusive","held":[h("outer:a","shared")],"request":q("outer:a","exclusive"),"expect":{"kind":"self-deadlock","heldClass":"outer","requestedClass":"outer"}},
  {"id":"self-deadlock-exclusive-then-shared","held":[h("outer:a","exclusive")],"request":q("outer:a","shared"),"expect":{"kind":"self-deadlock","heldClass":"outer","requestedClass":"outer"}},
  {"id":"bounded-self-conflict-is-left-to-expire","held":[h("outer:a","shared")],"request":q("outer:a","exclusive",False,D,True),"expect":None},
  {"id":"reentrant-shared-is-fine","held":[h("outer:a","shared")],"request":q("outer:a","shared"),"expect":None},
  {"id":"same-resource-other-directory-is-other-lease","held":[h("outer:a","exclusive",1,1000,"/other")],"request":q("outer:a","exclusive"),"expect":None},
  {"id":"oldest-offender-named","held":[h("inner:late","exclusive",1,3000),h("middle:early","exclusive",1,1000)],"request":q("outer:a"),"expect":{"kind":"out-of-order","held":"middle:early"}}
 ],
 "holdAndWait":[
  {"id":"within-grace","held":[h("outer:a")],"request":q("inner:b"),"waitedMilliseconds":999,"expect":None},
  {"id":"after-grace-refused","held":[h("outer:a")],"request":q("inner:b"),"waitedMilliseconds":1000,"blocker":{"pid":4242,"heldSeconds":77},"expect":{"kind":"hold-and-wait","held":"outer:a","blockerPid":4242,"blockerHeldSeconds":77}},
  {"id":"shared-hold-never-refused","held":[h("outer:a","shared")],"request":q("inner:b"),"waitedMilliseconds":99999,"expect":None},
  {"id":"declared-edge-exempt","held":[h("inner:a")],"request":q("middle:b"),"waitedMilliseconds":99999,"expect":None},
  {"id":"try-lock-exempt","held":[h("outer:a")],"request":q("inner:b","exclusive",True),"waitedMilliseconds":99999,"expect":None},
  {"id":"the-lease-itself-is-not-a-hold","held":[h("inner:b")],"request":q("inner:b","shared"),"waitedMilliseconds":99999,"expect":None},
  {"id":"unclassified-holds-count","held":[h("unknown")],"request":q("inner:b"),"waitedMilliseconds":2000,"expect":{"kind":"hold-and-wait","heldClass":"unclassified"}},
  {"id":"oldest-exclusive-named","held":[h("middle:new","exclusive",1,5000),h("outer:old","exclusive",1,1000)],"request":q("inner:b"),"waitedMilliseconds":2000,"expect":{"kind":"hold-and-wait","held":"outer:old"}}
 ],
 "cycles":[
  {"id":"two-party","from":"1/1","nodes":[{"id":"1/1","holds":[{"key":"A","mode":"exclusive"}],"wait":{"key":"B","mode":"exclusive","sinceMs":100,"queued":False}},{"id":"2/1","holds":[{"key":"B","mode":"exclusive"}],"wait":{"key":"A","mode":"exclusive","sinceMs":200,"queued":False}}],"expect":{"members":["1/1","2/1"],"victim":"2/1"}},
  {"id":"three-party-victim-is-latest-wait","from":"2/1","nodes":[{"id":"1/1","holds":[{"key":"A","mode":"exclusive"}],"wait":{"key":"B","mode":"exclusive","sinceMs":300,"queued":False}},{"id":"2/1","holds":[{"key":"B","mode":"exclusive"}],"wait":{"key":"C","mode":"exclusive","sinceMs":100,"queued":False}},{"id":"3/1","holds":[{"key":"C","mode":"exclusive"}],"wait":{"key":"A","mode":"exclusive","sinceMs":200,"queued":False}}],"expect":{"members":["2/1","3/1","1/1"],"victim":"1/1"}},
  {"id":"shared-holders-do-not-block-shared-waiters","from":"1/1","nodes":[{"id":"1/1","holds":[{"key":"A","mode":"shared"}],"wait":{"key":"B","mode":"shared","sinceMs":1,"queued":False}},{"id":"2/1","holds":[{"key":"B","mode":"shared"}],"wait":{"key":"A","mode":"shared","sinceMs":2,"queued":False}}],"expect":None},
  {"id":"chain-without-cycle","from":"1/1","nodes":[{"id":"1/1","holds":[],"wait":{"key":"A","mode":"exclusive","sinceMs":1,"queued":False}},{"id":"2/1","holds":[{"key":"A","mode":"exclusive"}],"wait":{"key":"B","mode":"exclusive","sinceMs":2,"queued":False}},{"id":"3/1","holds":[{"key":"B","mode":"exclusive"}]}],"expect":None},
  {"id":"queue-order-closes-a-cycle","from":"1/1","nodes":[{"id":"1/1","holds":[{"key":"A","mode":"shared"}],"wait":{"key":"B","mode":"exclusive","sinceMs":300,"queued":True}},{"id":"2/1","holds":[{"key":"B","mode":"shared"}],"wait":{"key":"A","mode":"exclusive","sinceMs":100,"queued":True}}],"expect":{"members":["1/1","2/1"],"victim":"1/1"}},
  {"id":"cycle-not-through-requester-is-left-to-its-members","from":"1/1","nodes":[{"id":"1/1","holds":[],"wait":{"key":"A","mode":"exclusive","sinceMs":1,"queued":False}},{"id":"2/1","holds":[{"key":"A","mode":"exclusive"},{"key":"C","mode":"exclusive"}],"wait":{"key":"B","mode":"exclusive","sinceMs":2,"queued":False}},{"id":"3/1","holds":[{"key":"B","mode":"exclusive"}],"wait":{"key":"C","mode":"exclusive","sinceMs":3,"queued":False}}],"expect":None}
 ],
 "messages":[
  {"id":"hold-and-wait-en","language":"en","violation":{"version":1,"kind":"hold-and-wait","held":"outer:a","heldMode":"exclusive","heldRank":1,"heldClass":"outer","requested":"inner:b","requestedMode":"shared","requestedRank":3,"requestedClass":"inner","waitedMilliseconds":15000,"blockerPid":4242,"blockerHeldSeconds":77,"atMs":1,"pid":1},"contains":["inner:b","shared","15 s","process 4242","77 s","outer:a","exclusively"]},
  {"id":"hold-and-wait-de","language":"de","violation":{"version":1,"kind":"hold-and-wait","held":"outer:a","heldMode":"exclusive","heldRank":1,"heldClass":"outer","requested":"inner:b","requestedMode":"shared","requestedRank":3,"requestedClass":"inner","waitedMilliseconds":15000,"blockerPid":4242,"blockerHeldSeconds":77,"atMs":1,"pid":1},"contains":["inner:b","15 s","Prozess 4242","77 s","outer:a","exklusiv"]},
  {"id":"order-en","language":"en","violation":{"version":1,"kind":"out-of-order","held":"inner:b","heldMode":"exclusive","heldRank":3,"heldClass":"inner","requested":"outer:a","requestedMode":"exclusive","requestedRank":1,"requestedClass":"outer","atMs":1,"pid":1},"contains":["Lock order violated","rank 1","rank 3","inner:b"]},
  {"id":"order-de","language":"de","violation":{"version":1,"kind":"out-of-order","held":"inner:b","heldMode":"exclusive","heldRank":3,"heldClass":"inner","requested":"outer:a","requestedMode":"exclusive","requestedRank":1,"requestedClass":"outer","atMs":1,"pid":1},"contains":["Sperrreihenfolge verletzt","Rang 1","Rang 3","inner:b"]},
  {"id":"cycle-de","language":"de","violation":{"version":1,"kind":"cycle","held":"a","heldMode":"exclusive","heldRank":0,"heldClass":"unclassified","requested":"b","requestedMode":"exclusive","requestedRank":0,"requestedClass":"unclassified","cycle":["1/1","2/1"],"atMs":1,"pid":1},"contains":["Verklemmung erkannt","1/1 → 2/1"]}
 ]
}
os.makedirs(os.path.dirname(OUT),exist_ok=True)
open(OUT,"w",encoding="utf8").write(json.dumps(fixture,ensure_ascii=False,indent=1)+"\n")
print("fixture ok")
