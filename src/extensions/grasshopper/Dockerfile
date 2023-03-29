# FROM mateusoliveira43/poetry:1.2.2-python3.10.10

# COPY . .
# RUN poetry install --no-root
# CMD ["poetry", "run", "main"]

# FROM python:3.10.10-slim

# ARG YOUR_ENV

# ENV YOUR_ENV=${YOUR_ENV} \
#   PYTHONFAULTHANDLER=1 \
#   PYTHONUNBUFFERED=1 \
#   PYTHONHASHSEED=random \
#   PIP_NO_CACHE_DIR=off \
#   PIP_DISABLE_PIP_VERSION_CHECK=on \
#   PIP_TIMEOUT=100 \
#   POETRY_VERSION=1.2.2

# # System deps:
# RUN pip install "poetry==$POETRY_VERSION"

# # Copy only requirements to cache them in docker layer
# WORKDIR /code
# COPY poetry.lock pyproject.toml /code/

# # Project initialization:
# RUN poetry config virtualenvs.create false \
#   && poetry install $(test "$YOUR_ENV" == production && echo "--no-dev") --no-interaction --no-ansi

# # Creating folders, and files for a project:
# COPY . /code

# FROM python:3.10.10-slim as base

# ENV PYTHONFAULTHANDLER=1 \
#     PYTHONHASHSEED=random \
#     PYTHONUNBUFFERED=1

# WORKDIR /app

# FROM base as builder

# ENV PIP_TIMEOUT=100 \
#     PIP_DISABLE_PIP_VERSION_CHECK=1 \
#     PIP_NO_CACHE_DIR=1 \
#     POETRY_VERSION=1.2.2

# RUN pip install "poetry==$POETRY_VERSION"
# RUN python -m venv /venv

# COPY pyproject.toml poetry.lock ./
# RUN poetry export -f requirements.txt | /venv/bin/pip install -r /dev/stdin

# COPY . .
# RUN poetry build && /venv/bin/pip install dist/*.whl

# FROM base as final

# COPY --from=builder /venv /venv
# CMD ["poetry", "run", "main"]