import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
a = s.index('rep(\'\'\'tag_a = self._say')
b = s.index("rep(\"self.step(\\\"towers\\\"")
new = '''rep(\'\'\'tag_a = self._say("A", build_b.cores[0].get_top() + np.array([0, 0.3, 0]), 24)
        tag_b = self._say("B", build_b.cores[1].get_top() + np.array([0, 0.3, 0]), 24)\'\'\', \'\'\'facade_count = lambda t: len(_nak.facade_storeys(t)[0])
        tag_a = self._say("A  " + g(f"{self.A.capsule_storeys} storeys, {facade_count(self.A)} facades", f"{self.A.capsule_storeys} Geschosse, {facade_count(self.A)} Fassaden"), build_b.cores[0].get_top() + np.array([-0.2, 0.35, 0]), 18, tint_a)
        tag_b = self._say("B  " + g(f"{self.B.capsule_storeys} storeys, {facade_count(self.B)} facades", f"{self.B.capsule_storeys} Geschosse, {facade_count(self.B)} Fassaden"), build_b.cores[1].get_top() + np.array([0.3, 0.35, 0]), 18, tint_b)\'\'\')
'''
s = s[:a] + new + s[b:]
s = s.replace("self._pt(closeup, *(centre + np.array([self.B.center[0], 0, 0])))", "self._pt(closeup, *centre)")
open(p, "w", encoding="utf-8").write(s)
