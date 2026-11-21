"""🧭 Round 5: scans transport poses of the excavator (boom chord angle for a given stick fold and bucket curl so that the bucket rests just above the ground) and prints the heights of the top pins and the length, against the Cat 320 data sheet (machine height with boom, stick and bucket 3160 mm, length 9530 mm)."""
import importlib.util, os, sys
here = os.path.dirname(os.path.abspath(__file__))
sys.argv = [sys.argv[0]]
spec = importlib.util.spec_from_file_location("b", os.path.join(here, "r3-excavator-build.py"))
B = importlib.util.module_from_spec(spec); spec.loader.exec_module(B)
K, F, L = B.K, B.F, B.L
for sg in (-136.0, -139.0):
    for be in (-100.0, -120.0, -140.0):
        for clr in (0.12, 0.05, 0.02):
            lo, hi = -10.0, 30.0
            for _ in range(50):
                m = (lo + hi) / 2
                if B.bucket_low(m, sg, be) < clr: lo = m
                else: hi = m
            th = (lo + hi) / 2
            p = K.chain(th, sg, be)
            boomtop = max(K.add(F.PIN_A if hasattr(F, "PIN_A") else K.PIN["A"], K.rot(q, th))[1] for q in L.poly(F.BOOM).pts(0.01))
            sticktop = max(K.add(p["B"], K.rot(q, th + sg))[1] for q in L.poly(F.STICK).pts(0.01))
            front = max(max(K.add(p["B"], K.rot(q, th + sg))[0] for q in L.poly(F.STICK).pts(0.01)), max(K.add(K.PIN["A"], K.rot(q, th))[0] for q in L.poly(F.BOOM).pts(0.01)))
            print("stick %6.1f bucket %6.1f clr %.2f boom %6.2f  E %.3f F %.3f B %.3f boomtop %.3f sticktop %.3f front x %.3f" % (sg, be, clr, th, p["E"][1] + K.DECK, p["F"][1] + K.DECK, p["B"][1] + K.DECK, boomtop + K.DECK, sticktop + K.DECK, front))
