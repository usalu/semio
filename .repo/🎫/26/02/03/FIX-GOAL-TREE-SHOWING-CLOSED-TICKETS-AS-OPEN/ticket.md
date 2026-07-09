# Ticket

## Summary

Fixed case-sensitive status comparison in formatGoalTree function. The GraphQL API returns status in uppercase (CLOSED) but the comparison was checking for lowercase (closed). Used strings.ToLower() for case-insensitive comparison.

## Changes

## Log

## Todos

## Plan
