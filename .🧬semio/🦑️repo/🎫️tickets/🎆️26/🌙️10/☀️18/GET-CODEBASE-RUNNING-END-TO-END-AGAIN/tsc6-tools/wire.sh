python3 -c "
import json,time
print(json.dumps({'version':1,'policy':{'version':1,'owner':'tsc6:test','maximumElapsedMilliseconds':900000},'deadlineEpochMilliseconds':int(time.time()*1000)+890000,'capabilities':{}}))"
