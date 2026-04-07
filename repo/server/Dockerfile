# #region 🔖Header
# [🧰repo⌨️server💻Dockerfile](repo://p/i/repo/b/b/server/f/Dockerfile)
# 2025 Ueli Saluz <ueli@semio-tech.com>
# AGPL-3.0
# Multi-stage Dockerfile for the Next.js repo server.
# #endregion 🔖Header

# #region 🔖Build
FROM node:22-alpine AS base

FROM base AS deps
WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm ci --omit=dev

FROM base AS builder
WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm ci
COPY . .
RUN npm run build
# #endregion 🔖Build

# #region 🔖Runtime
FROM base AS runner
WORKDIR /app
ENV NODE_ENV=production

RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs
EXPOSE 8787
ENV PORT=8787
ENV HOSTNAME="0.0.0.0"

CMD ["node", "server.js"]
# #endregion 🔖Runtime
