from argparse import ArgumentParser

parser = ArgumentParser(
    "semio",
    description="For working with semio."
)
parser.add_argument(
    '-d',
    '--directory',
    action='store',
    nargs="?",
    required= False,
    help='Provide a directory if it the archive us not in the current one.')


subparsers = parser.add_subparsers()

addParser = subparsers.add_parser('add', help='Add an artifact to the archive')



parser.parse_args()
