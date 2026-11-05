# Parallel BIM Beside Anatomy

The six English BIM lessons each have one segment and those segments occur only in that lesson. They do not share media directories with grasshopper anatomy (`s/120`) or the optimization lessons (`s/129`–`s/134`).

`parallel-bim.ps1` renders `bim_information`, `bim_objects`, `bim_representations`, `bim_models`, `bim_exchange`, and `bim_consistency` into `cache/videos`. It does not take `videos/en/.media-lock` and it does not write `generated/render-bim.log`. The waiting `render-bim.ps1` still owns `energy_balance` and `cooling`, then repeats the BIM lessons after the lock clears. A repeated BIM lesson should hit the fresh master and return without Manim. `join-en.ps1` still waits for `bim_consistency ok` in `render-bim.log` and for the lock to disappear.

At most four BIM lessons run at once, and only while more than 8 GB of RAM is free. Anatomy stays untouched.
