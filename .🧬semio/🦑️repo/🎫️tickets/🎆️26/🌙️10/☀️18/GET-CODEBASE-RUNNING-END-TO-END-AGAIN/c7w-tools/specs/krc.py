from specs.lib import *
F = RW + '/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🦀️.rs'
c = Cut(F, 'krc.rs')
c.child('🔁', 'spawn-recovery', 'spawn_and_recovery', [(4, 351)],
        'Spawn job and recovery: the replay constants, the raw spawn job owner and the mounted product replay recovery token, owner and registry.')
c.child('🚫', 'request-refusal', 'request_and_refusal', [(353, 652)],
        'Request and refusal: the mounted product replay request and the refusal token, cause, phase, retained refusal and refusal registry.')
c.child('🛂', 'admission-claim', 'admission_and_claim', [(654, 951)],
        'Admission and claim: the admission permit, the refused replay, the admission, checkpoint and claim, the expected record and the replay authority.')
c.child('📄', 'typed-result', 'typed_result_and_exchange', [(953, 1172)],
        'Typed result and exchange: the typed operation result page, the exchange surface document and outcome, the kernel outcome and the response slot.')
SPEC = c.spec()
