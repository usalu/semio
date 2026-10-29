import runpy, sys
S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/"
for case in ["🌡️blackbody-exchange", "💧️vapour-pressure", "🧍️clothed-human", "🍃️leaf-transpiration", "🫁️breath-plume", "🚗️exhaust-wind"]:
    sys.argv = ["🐍️.py"] + sys.argv[1:]
    runpy.run_path(S + case + "/🐍️.py", run_name="__main__")
