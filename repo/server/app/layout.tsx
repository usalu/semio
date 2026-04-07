// #region 🔖Header
// [🧰repo⌨️server🛅app💻layout](repo://p/i/repo/b/b/server/f/app/layout.tsx)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Root layout for the repo server web app.
// #endregion 🔖Header

export const metadata = {
  title: "semio repo",
  description: "Monorepo management server",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
