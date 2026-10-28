import re, sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
s = open(p, encoding="utf-8").read()
a = s.index("    @staticmethod\n    def _eta")
b = s.index("    def _demand")
s = s[:a] + s[b:]
old_start = s.index("            envelope, gains, eta = self._flows")
old_end = s.index("            names = ((self.tr(\"Losses\"")
s = s[:old_start] + '''            first, second, eta = ((found["heating_losses"], found["heating_gains"], found["heating_eta"]) if winter else (found["cooling_gains"], found["cooling_removable"], found["cooling_eta"]))
            demand = found[part + "_demand"]
            sign = "=" if half_up(half_up(first) - half_up(eta, 2) * half_up(second)) == half_up(demand) else "≈"
''' + s[old_end:]
s = s.replace('self.live(first, soft, 0, "kWh/a"), "−", self.live(eta, INK, 2), "×", self.live(second, soft, 0, "kWh/a")',
              'self.live(lambda d, f: first_of(f), soft, 0, "kWh/a"), "−", self.live(lambda d, f: eta_of(f), INK, 2), "×", self.live(lambda d, f: second_of(f), soft, 0, "kWh/a")')
open(p, "w", encoding="utf-8").write(s)
