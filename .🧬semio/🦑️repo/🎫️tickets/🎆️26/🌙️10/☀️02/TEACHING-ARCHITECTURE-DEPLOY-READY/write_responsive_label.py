import os

chrome = open(os.path.join(os.environ["TEMP"], "semio-chrome-path.txt"), encoding="utf-8").read().strip()
folder = os.path.join(chrome, "responsive-label")
os.makedirs(folder, exist_ok=True)
path = os.path.join(folder, "\U0001f7e6\ufe0f.tsx")
text = """import * as React from \"react\";

/** \U0001f4f1 A shell label that shows its short form below the tablet breakpoint and its full form from tablet width up. Both forms stay available to assistive technology. Lives beside the slim chrome so a page that imports `@semio-tech/ui-react/chrome` does not load the React target barrel.
 * @see ../\U0001f7e6\ufe0f.ts — the slim chrome that re-exports this
 * @see ../../\U0001f7e6\ufe0f.tsx — the full React target, which re-exports it unchanged
 */
export const shellChromeTitleClassName = \"truncate text-sm font-medium text-element\";

/** \U0001f4f1 The short form below the tablet breakpoint, the full form from tablet width up. */
export function ResponsiveLabel(props: { readonly full: string; readonly short?: string; readonly className?: string; readonly title?: string }): React.ReactElement {
  const short = props.short ?? props.full;
  if (short === props.full) return <span className={props.className} title={props.title ?? props.full}>{props.full}</span>;
  return (
    <span className={props.className} title={props.title ?? props.full}>
      <span aria-hidden=\"true\" className=\"md:hidden\">{short}</span>
      <span className=\"max-md:sr-only\">{props.full}</span>
    </span>
  );
}
"""
open(path, "w", encoding="utf-8", newline="\n").write(text)
print(path)
print(os.path.getsize(path))
