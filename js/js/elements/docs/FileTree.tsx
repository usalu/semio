// #region Header

// FileTree.tsx

// 2025 Ueli Saluz

// #endregion

import { File, Folder } from "lucide-react";
import { FC, ReactNode } from "react";

export interface FileTreeProps {
  children: ReactNode;
}

export const FileTree: FC<FileTreeProps> = ({ children }) => {
  return (
    <div className="my-4 p-4 bg-gray-50 dark:bg-gray-900 rounded border border-gray-200 dark:border-gray-800 font-mono text-sm">
      <ul className="list-none pl-0 space-y-1">{children}</ul>
    </div>
  );
};

export interface FileTreeItemProps {
  name: string;
  type?: "file" | "folder";
}

export const FileTreeItem: FC<FileTreeItemProps> = ({ name, type = "file" }) => {
  const Icon = type === "folder" ? Folder : File;
  return (
    <li className="flex items-center gap-2 py-0.5">
      <Icon className="w-4 h-4" />
      <span>{name}</span>
    </li>
  );
};
