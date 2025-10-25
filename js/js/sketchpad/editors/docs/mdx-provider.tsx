import { MDXProvider as BaseMDXProvider } from "@mdx-js/react";
import { FC, ReactNode } from "react";
import { Aside } from "../../../elements/docs/Aside";
import { Card, CardGrid } from "../../../elements/docs/Card";
import { FileTree, FileTreeItem } from "../../../elements/docs/FileTree";
import Section from "../../../elements/docs/Section";
import { Steps } from "../../../elements/docs/Steps";
import { TabItem, Tabs } from "../../../elements/docs/Tabs";

const components = {
  Card,
  CardGrid,
  Steps,
  Tabs,
  TabItem,
  Aside,
  FileTree,
  FileTreeItem,
  Section,
  h1: ({ children, id, ...props }: any) => (
    <h1 id={id} className="text-4xl font-bold mb-4 mt-8" {...props}>
      {children}
    </h1>
  ),
  h2: ({ children, id, ...props }: any) => (
    <h2 id={id} className="text-3xl font-semibold mb-3 mt-6" {...props}>
      {children}
    </h2>
  ),
  h3: ({ children, id, ...props }: any) => (
    <h3 id={id} className="text-2xl font-semibold mb-2 mt-5" {...props}>
      {children}
    </h3>
  ),
  h4: ({ children, id, ...props }: any) => (
    <h4 id={id} className="text-xl font-semibold mb-2 mt-4" {...props}>
      {children}
    </h4>
  ),
  h5: ({ children, id, ...props }: any) => (
    <h5 id={id} className="text-lg font-semibold mb-1 mt-3" {...props}>
      {children}
    </h5>
  ),
  h6: ({ children, id, ...props }: any) => (
    <h6 id={id} className="text-base font-semibold mb-1 mt-2" {...props}>
      {children}
    </h6>
  ),
  p: ({ children, ...props }: any) => (
    <p className="mb-4 leading-7" {...props}>
      {children}
    </p>
  ),
  a: ({ children, href, ...props }: any) => (
    <a href={href} className="text-primary hover:underline" {...props}>
      {children}
    </a>
  ),
  ul: ({ children, ...props }: any) => (
    <ul className="list-disc list-inside mb-4 space-y-2" {...props}>
      {children}
    </ul>
  ),
  ol: ({ children, ...props }: any) => (
    <ol className="list-decimal list-inside mb-4 space-y-2" {...props}>
      {children}
    </ol>
  ),
  li: ({ children, ...props }: any) => (
    <li className="ml-4" {...props}>
      {children}
    </li>
  ),
  code: ({ children, className, ...props }: any) => {
    const inline = !className;
    if (inline) {
      return (
        <code className="bg-gray-100 dark:bg-gray-800 px-1.5 py-0.5 rounded text-sm font-mono" {...props}>
          {children}
        </code>
      );
    }
    return (
      <code className={`block bg-gray-100 dark:bg-gray-800 p-4 rounded overflow-x-auto font-mono text-sm ${className}`} {...props}>
        {children}
      </code>
    );
  },
  pre: ({ children, ...props }: any) => (
    <pre className="bg-gray-100 dark:bg-gray-800 p-4 rounded overflow-x-auto mb-4" {...props}>
      {children}
    </pre>
  ),
  blockquote: ({ children, ...props }: any) => (
    <blockquote className="border-l-4 border-gray-300 dark:border-gray-700 pl-4 italic my-4" {...props}>
      {children}
    </blockquote>
  ),
  hr: (props: any) => <hr className="my-8 border-gray-300 dark:border-gray-700" {...props} />,
  img: ({ src, alt, ...props }: any) => <img src={src} alt={alt} className="max-w-full h-auto rounded my-4" {...props} />,
  table: ({ children, ...props }: any) => (
    <div className="overflow-x-auto my-4">
      <table className="min-w-full border-collapse border border-gray-300 dark:border-gray-700" {...props}>
        {children}
      </table>
    </div>
  ),
  thead: ({ children, ...props }: any) => (
    <thead className="bg-gray-100 dark:bg-gray-800" {...props}>
      {children}
    </thead>
  ),
  tbody: ({ children, ...props }: any) => <tbody {...props}>{children}</tbody>,
  tr: ({ children, ...props }: any) => (
    <tr className="border-b border-gray-300 dark:border-gray-700" {...props}>
      {children}
    </tr>
  ),
  th: ({ children, ...props }: any) => (
    <th className="px-4 py-2 text-left font-semibold" {...props}>
      {children}
    </th>
  ),
  td: ({ children, ...props }: any) => (
    <td className="px-4 py-2" {...props}>
      {children}
    </td>
  ),
};

interface MDXProviderProps {
  children: ReactNode;
}

export const MDXProvider: FC<MDXProviderProps> = ({ children }) => {
  return <BaseMDXProvider components={components}>{children}</BaseMDXProvider>;
};
