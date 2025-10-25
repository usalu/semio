// #region Header

// Tabs.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, ReactNode } from "react";
import { Tabs as BaseTabs, TabsContent, TabsList, TabsTrigger } from "../aggregation/Tabs";

export interface TabItemProps {
  label: string;
  children: ReactNode;
}

export const TabItem: FC<TabItemProps> = ({ children }) => {
  return <>{children}</>;
};

export interface TabsProps {
  children: ReactNode;
}

export const Tabs: FC<TabsProps> = ({ children }) => {
  const items = Array.isArray(children) ? children : [children];
  const tabItems = items.filter((child: any) => child?.type === TabItem);

  if (tabItems.length === 0) return <div className="tabs-container">{children}</div>;

  return (
    <BaseTabs defaultValue={tabItems[0]?.props?.label || "0"} className="my-4">
      <TabsList>
        {tabItems.map((item: any, idx: number) => (
          <TabsTrigger key={idx} value={item.props.label || idx.toString()}>
            {item.props.label}
          </TabsTrigger>
        ))}
      </TabsList>
      {tabItems.map((item: any, idx: number) => (
        <TabsContent key={idx} value={item.props.label || idx.toString()}>
          {item.props.children}
        </TabsContent>
      ))}
    </BaseTabs>
  );
};
