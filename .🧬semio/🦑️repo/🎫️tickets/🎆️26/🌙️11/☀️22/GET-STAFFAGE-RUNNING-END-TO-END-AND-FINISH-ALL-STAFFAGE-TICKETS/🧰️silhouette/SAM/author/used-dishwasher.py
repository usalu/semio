"""🍽️ Repair ops of `used-dishwasher`: the coiled drain and inlet hose at the back are tori of 24 and 20 mm tube declared `slender`; the guard compares a slender solid at the pen of a stroke, which widened the two coils until they closed the 8 mm between them and stood 6 mm proud of the drawn coils (1:50 elevation 26.1 mm). At 1:50 a 20 mm tube is well above the smallest feature of 7.5 mm, so the two coils are compared as the bodies they are (no `slender` at 1:50); the drawing was right and is unchanged."""
from oplib import write

write("used-dishwasher", [{"op": "set", "path": ["volumes", "50", "body", {"id": i}, "slender"]} for i in ("drain-coil", "inlet-coil")])
