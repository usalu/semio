# Parallel Ahead

`excel_energy` and `excel_program` had grown to about 10 GB and 9 GB while each used under one core. Free RAM was about 2.5 GB, so anatomy and the six BIM renders were sharing a paging machine. Those two Excel lessons are last in the serial queue, after capsule, casting, trees, excel anatomy, excel references, and tower.

Their repo process trees were stopped. `parallel-ahead.ps1` then admits the earlier lessons one at a time, and only while more than 14 GB stays free, so the next scene is not started until the previous one has allocated. It does not take `.media-lock` and does not write `render-en.log`. If that log records an attempt for a lesson this driver started, the driver stops only that lesson's process tree.
