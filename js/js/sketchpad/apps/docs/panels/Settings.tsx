// #region Header

// Settings.tsx

// 2025 Ueli Saluz

// #endregion

import { FC } from "react";

interface SettingsProps {}

const Settings: FC<SettingsProps> = () => {
  return (
    <div className="p-4">
      <h3 className="text-sm font-semibold mb-2">Documentation Settings</h3>
      <p className="text-xs text-muted-foreground">Settings for documentation display and preferences.</p>
    </div>
  );
};

export default Settings;
