from lib import *
def patch_ready3(t):
    t=once(t,'"stalled"|"code-changed"|"adopted"|"binary-missing";','"stalled"|"code-changed"|"code-stale"|"adopted"|"binary-missing";',"reason type")
    t=once(t,'stalledReason=byCode?"code-changed":"stalled";','stalledReason=stale?"code-stale":byCode?"code-changed":"stalled";',"reason pick")
    return t
def patch_ready_schema3(t):
    t=once(t,'"code-changed": "Stage {stage} waited more than 10 minutes for a named holder while its code changed and was stopped; started again at once with the new code.",','"code-changed": "Stage {stage} waited more than 10 minutes for a named holder while its code changed and was stopped; started again at once with the new code.", "code-stale": "Stage {stage} had run for at least 30 minutes while its code changed and was stopped; started again at once with the new code, what it finished stays stored.",',"en")
    t=once(t,'"code-changed": "Stufe {stage} wartete mehr als 10 Minuten auf einen benannten Halter, während sich ihr Code änderte, und wurde beendet; sofort neu gestartet mit dem neuen Code.",','"code-changed": "Stufe {stage} wartete mehr als 10 Minuten auf einen benannten Halter, während sich ihr Code änderte, und wurde beendet; sofort neu gestartet mit dem neuen Code.", "code-stale": "Stufe {stage} lief mindestens 30 Minuten, während sich ihr Code änderte, und wurde beendet; sofort neu gestartet mit dem neuen Code, alles Fertige bleibt gespeichert.",',"de")
    t=once(t,'"stalled", "code-changed", "adopted", "binary-missing"]','"stalled", "code-changed", "code-stale", "adopted", "binary-missing"]',"enum")
    return t
def patch_ready_test3(t):
    return once(t,'const restarted=w.updates.filter(update=>update.state==="running"&&update.stage==="tools");\n expect(restarted[1]!.reason!.code).toBe("code-changed");\n});','const restarted=w.updates.filter(update=>update.state==="running"&&update.stage==="tools");\n expect(restarted[1]!.reason!.code).toBe("code-stale");expect(restarted[1]!.reason!.en).toContain("30 minutes");expect(restarted[1]!.reason!.de).toContain("30 Minuten");expect(restarted[1]!.reason!.de).toContain("neuen Code");\n});',"test")
TARGETS3={READY:patch_ready3,READY_SCHEMA:patch_ready_schema3,READY_TEST:patch_ready_test3}
