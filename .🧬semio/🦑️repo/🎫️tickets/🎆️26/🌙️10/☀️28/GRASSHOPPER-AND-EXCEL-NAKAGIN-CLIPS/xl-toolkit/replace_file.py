import os, sys, time
src, dst = sys.argv[1], sys.argv[2]
for attempt in range(20):
    try:
        os.replace(src, dst); print("replaced"); break
    except OSError as error:
        print("retry", attempt, error); time.sleep(3)
