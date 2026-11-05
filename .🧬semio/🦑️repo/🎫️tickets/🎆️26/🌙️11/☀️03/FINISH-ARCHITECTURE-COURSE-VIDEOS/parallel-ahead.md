# Parallel Ahead

excel_energy and excel_program had grown to about 10 GB and 9 GB while each used under one core. Free RAM was about 2 GB, so anatomy and the six BIM renders were paging. Those two lessons are last in the serial queue.

Their repo process trees were stopped. Free RAM returned to about 20 GB. Anatomy and the six BIM renders stayed up. parallel-ahead.ps1 then admits capsule, casting, trees, excel anatomy, excel references, and tower, one start at a time, and only while more than 14 GB stays free. excel_program and excel_energy are last in that queue so they restart only after memory is free. The driver does not take the English media lock and does not write render-en.log. A serial attempt for one of its lessons stops only that lesson's process tree.

Casting frames landed in about 1.4 seconds, and those scenes stayed under 1 GB, so the driver cap was raised to six. Tower, excel_program, and excel_energy still wait for 18 GB free. Adopted the live capsule, casting, trees, and excel_anatomy renders, then started excel_references.
