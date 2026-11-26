# Excel annual result capture

The annual result scene shows the complete `148 kWh/(m²·a)` in B15 in English and German. Manim's actual capture has the worksheet's authored 10% green reference fill over the text and paper. On the antialiased edge of the leading `1`, that fill changes `[114, 114, 114]` to `[115, 121, 115]`. The former fixed `<120` test rejected this one pixel. The frame handles are outside the text region.

The language-neutral fixture now declares the authored fill opacity. The result test independently composites the Excel reference fill over its typeset baseline with Pillow and compares the entire visible B15 label against the Manim capture. The match is within one channel level in both languages. Erasing the first digit in the captured pixels introduces a 202-level error in either language.

The regression reproduced through the repository's managed Python environment before the test change. The corrected focused test passed through the same entry point, covering three stages in both languages.
